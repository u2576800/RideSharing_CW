import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { ZoyaProfile } from "../target/types/zoya_profile";
import { ZoyaRide } from "../target/types/zoya_ride";

describe("zoya", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const profileProgram = anchor.workspace.ZoyaProfile as Program<ZoyaProfile>;
  const rideProgram = anchor.workspace.ZoyaRide as Program<ZoyaRide>;

  const driver = Keypair.generate();
  const rider = Keypair.generate();

  let driverProfilePDA: PublicKey;
  let riderProfilePDA: PublicKey;
  let rideRequestPDA: PublicKey;

  const fundKeypair = async (kp: Keypair, sol = 2) => {
    const sig = await provider.connection.requestAirdrop(kp.publicKey, sol * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig, "confirmed");
  };

  before(async () => {
    await fundKeypair(driver);
    await fundKeypair(rider);

    [driverProfilePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("driver_profile"), driver.publicKey.toBuffer()],
      profileProgram.programId
    );
    [riderProfilePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("rider_profile"), rider.publicKey.toBuffer()],
      profileProgram.programId
    );
  });

  it("registers a driver", async () => {
    await profileProgram.methods
      .registerDriver("Alice", "Toyota Camry KA01AB1234", "+91-9999999999")
      .accounts({
        driverProfile: driverProfilePDA,
        authority: driver.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([driver])
      .rpc();

    const profile = await profileProgram.account.driverProfile.fetch(driverProfilePDA);
    assert.equal(profile.name, "Alice");
    assert.equal(profile.vehicleInfo, "Toyota Camry KA01AB1234");
    assert.equal(profile.totalRidesCompleted.toNumber(), 0);
    assert.equal(profile.rating, 500);
  });

  it("registers a rider", async () => {
    await profileProgram.methods
      .registerRider("Bob", "+91-8888888888")
      .accounts({
        riderProfile: riderProfilePDA,
        authority: rider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([rider])
      .rpc();

    const profile = await profileProgram.account.riderProfile.fetch(riderProfilePDA);
    assert.equal(profile.name, "Bob");
    assert.equal(profile.totalRides.toNumber(), 0);
  });

  it("creates a ride request", async () => {
    const riderProfile = await profileProgram.account.riderProfile.fetch(riderProfilePDA);
    const rideIndex = riderProfile.totalRides;

    const indexBuf = Buffer.alloc(8);
    indexBuf.writeBigUInt64LE(BigInt(rideIndex.toString()));

    [rideRequestPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("ride_request"), rider.publicKey.toBuffer(), indexBuf],
      rideProgram.programId
    );

    await rideProgram.methods
      .createRideRequest(
        "Brigade Road, Bangalore",
        "Indiranagar, Bangalore",
        new BN(0.05 * LAMPORTS_PER_SOL)
      )
      .accounts({
        rideRequest: rideRequestPDA,
        riderProfile: riderProfilePDA,
        rider: rider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([rider])
      .rpc();

    const ride = await rideProgram.account.rideRequest.fetch(rideRequestPDA);
    assert.equal(ride.pickupAddress, "Brigade Road, Bangalore");
    assert.equal(ride.dropoffAddress, "Indiranagar, Bangalore");
    assert.deepEqual(ride.status, { pending: {} });
  });

  it("driver accepts the ride", async () => {
    await rideProgram.methods
      .acceptRide()
      .accounts({
        rideRequest: rideRequestPDA,
        driverProfile: driverProfilePDA,
        driver: driver.publicKey,
      })
      .signers([driver])
      .rpc();

    const ride = await rideProgram.account.rideRequest.fetch(rideRequestPDA);
    assert.deepEqual(ride.status, { accepted: {} });
    assert.equal(ride.driver.toBase58(), driver.publicKey.toBase58());
  });

  it("driver starts the ride", async () => {
    await rideProgram.methods
      .startRide()
      .accounts({
        rideRequest: rideRequestPDA,
        driver: driver.publicKey,
      })
      .signers([driver])
      .rpc();

    const ride = await rideProgram.account.rideRequest.fetch(rideRequestPDA);
    assert.deepEqual(ride.status, { inProgress: {} });
  });

  it("driver completes the ride — CPI bumps total_rides_completed", async () => {
    const beforeProfile = await profileProgram.account.driverProfile.fetch(driverProfilePDA);
    assert.equal(beforeProfile.totalRidesCompleted.toNumber(), 0);

    await rideProgram.methods
      .completeRide()
      .accounts({
        rideRequest: rideRequestPDA,
        driverProfile: driverProfilePDA,
        driver: driver.publicKey,
        zoyaProfileProgram: profileProgram.programId,
      })
      .signers([driver])
      .rpc();

    const ride = await rideProgram.account.rideRequest.fetch(rideRequestPDA);
    assert.deepEqual(ride.status, { completed: {} });

    const afterProfile = await profileProgram.account.driverProfile.fetch(driverProfilePDA);
    assert.equal(
      afterProfile.totalRidesCompleted.toNumber(),
      1,
      "CPI failed: total_rides_completed should have incremented to 1"
    );
  });

  it("cannot cancel a completed ride", async () => {
    try {
      await rideProgram.methods
        .cancelRide()
        .accounts({
          rideRequest: rideRequestPDA,
          signer: rider.publicKey,
        })
        .signers([rider])
        .rpc();
      assert.fail("expected cancel to fail on completed ride");
    } catch (e: any) {
      assert.include(e.toString(), "CannotCancelInProgress");
    }
  });
});
