<script>
  let waveList = [];
  import Header from "./components/Header.svelte";
  import Bio from "./components/Bio.svelte";
  import Wallet from "./components/Wallet.svelte";
  import SendWave from "./components/SendWave.svelte";
  import WaveList from "./components/WaveList.svelte";
  import { onMount } from "svelte";
  import { ethers } from "ethers";
  import WavePortal from "../artifacts/contracts/WavePortal.sol/WavePortal.json";

  const CONTRACT_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

  async function getAllWaves() {
    if (!window.ethereum) {
      // If MetaMask is not installed, we use the default provider,
      // which is backed by a variety of third-party services (such
      // as INFURA). They do not have private keys installed so are
      // only have read-only access
      console.log("MetaMask not installed");
      return;
    }

    //const provider = new ethers.providers.Web3Provider(window.ethereum);
    const provider = new ethers.BrowserProvider(window.ethereum)

    const wavePortalContract = new ethers.Contract(
      CONTRACT_ADDRESS,
      WavePortal.abi,
      provider
    );
    const recievedWaves = await wavePortalContract.getAllWaves();

    if (!recievedWaves) {
      waveList = [];
      return;
    }

    const normalizeWave = (wave) => ({
      reaction: wave.reaction,
      message: wave.message,
      waver: wave.waver,
      //timestamp: new Date(wave.timestamp * 1000),
      timestamp: new Date(Number(wave.timestamp * 1000n)),
    });

    waveList = recievedWaves
      .map(normalizeWave)
      .sort((a, b) => b.timestamp - a.timestamp);
    console.log("waveList: ", waveList);
    return;
  }

  onMount(getAllWaves);
</script>

<main>
  <Header />
  <Bio />
  <Wallet />
  <SendWave {CONTRACT_ADDRESS} fetchWaves={getAllWaves} />
  <WaveList {waveList} />
</main>

<style>
</style>
