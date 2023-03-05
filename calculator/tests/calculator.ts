import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Calculator } from "../target/types/calculator";
const { SystemProgram } = anchor.web3
import { expect } from 'chai';

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  //Referencing the program - Abstraction that allows us to call methods of our SOL program
  const program = anchor.workspace.Calculator as Program<Calculator>;
  const programProvider = program.provider as anchor.AnchorProvider;

  //Generate keypair
  const calculatorPair = anchor.web3.Keypair.generate();

  //Program variable to pass param
  const initMsg = "calculator tests";

  it("Creating Calculator Instance", async () => {
    //Set our calculator keypair as a signer
    await program.methods.create(initMsg).accounts({
        calculator: calculatorPair.publicKey,
        user: programProvider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      } 
    ).signers([calculatorPair]).rpc();

    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.greeting).to.eql(initMsg);
  });

  it('Addition',async () => {
    await program.methods.add(new anchor.BN(2), new anchor.BN(3)).accounts({calculator: calculatorPair.publicKey}).rpc();
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(5));
  })

  it('Substraction',async () => {
    await program.methods.sub(new anchor.BN(2), new anchor.BN(3)).accounts({calculator: calculatorPair.publicKey}).rpc();
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(-1));
  })

  it('Multiplication',async () => {
    await program.methods.mul(new anchor.BN(2), new anchor.BN(3)).accounts({calculator: calculatorPair.publicKey}).rpc();
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(6));
  })

  it('Division',async () => {
    await program.methods.div(new anchor.BN(42), new anchor.BN(2)).accounts({calculator: calculatorPair.publicKey}).rpc();
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(21));
  })

  it('Division by zero',async () => {
    await program.methods.div(new anchor.BN(42), new anchor.BN(0)).accounts({calculator: calculatorPair.publicKey}).rpc();
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(0));
  })

});