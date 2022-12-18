import { ethers } from "ethers";
import { Window, StdABI, StdEventsABI } from "./types"
import Std from "../assets/Student.json";
import Ast from "../assets/ASTEvent.json";

declare let window: Window;

export class AstContextBuilder {
  provider: ethers.providers.Web3Provider | undefined;
  signer: ethers.providers.JsonRpcSigner | undefined;
  student: ethers.Contract | undefined;
  stdAddr: string;
  stdABI: StdABI;
  stdEventsABI: StdEventsABI;
  eventAddr: string;
  stdEvents: ethers.Contract | undefined;
  address: string | undefined;
  constructor() {
    this.stdAddr = "0x17Ba1d80C4802c0a9FB8F1084d7752754cA63d7f";
    this.eventAddr = "0x7eD660c870046474FE5Ebb96C6A2Ebc3109d0dd2";
    this.stdABI = Std.abi;
    this.stdEventsABI = Ast.abi;
  }
  async addProvider(): Promise<AstContextBuilder> {
    this.provider = new ethers.providers.Web3Provider(window.ethereum);
    await this.provider.send("eth_requestAccounts", []);
    return this;
  }
  async addSigner(): Promise<AstContextBuilder> {
    this.signer = this.provider?.getSigner();
    this.address = await this.signer?.getAddress();
    return this;
  }
  async addContracts(): Promise<AstContextBuilder> {
    this.student = new ethers.Contract(this.stdAddr, this.stdABI, this.signer);
    this.stdEvents = new ethers.Contract(this.stdAddr, this.stdEventsABI, this.signer);
    return this;
  }
  build(): AstContext {
    return new AstContext(this.provider, this.signer, this.student, this.stdEvents);
  }
}
export class AstContext {

  provider: ethers.providers.Web3Provider | undefined;
  signer: ethers.providers.JsonRpcSigner | undefined;
  student: ethers.Contract | undefined;
  stdEvents: ethers.Contract | undefined;
  constructor(provider: ethers.providers.Web3Provider | undefined,
    signer: ethers.providers.JsonRpcSigner | undefined,
    student: ethers.Contract | undefined,
    stdEvents: ethers.Contract | undefined) {
    this.provider = provider;
    this.signer = signer;
    this.student = student;
    this.stdEvents = stdEvents;
  }
}

