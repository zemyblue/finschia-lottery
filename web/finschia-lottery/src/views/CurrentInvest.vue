<template>
  <div class="container">
    <div class="row">
      <div class="col-7">
        <div class="card">
          <div class="card-header m-1">
            <div class="row mt-2">
              <h5 class="col text-start align-middle">Round {{ investment.round }}</h5><h5 class="col text-end align-middle">Total:
              {{ investment.total_amount }}
              {{ denom }}</h5>
            </div>
          </div>
          <div class="card-body">
            <table class="table">
              <thead>
              <tr>
                <th scope="col">Address</th>
                <th scope="col">Amount</th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="investor in this.investment.investors" :key="investor">
                <td>{{ investor.addr }}</td>
                <td>{{ investor.amount }} {{ denom }}</td>
              </tr>
              </tbody>
            </table>
<!--            <div>-->
<!--              <nav aria-label="Page navigation example">-->
<!--                <ul class="pagination justify-content-center">-->
<!--                  <li class="page-item"><a class="page-link" href="#">&laquo;</a></li>-->
<!--                  <li class="page-item"><a class="page-link" href="#">1</a></li>-->
<!--                  <li class="page-item"><a class="page-link" href="#">2</a></li>-->
<!--                  <li class="page-item"><a class="page-link" href="#">3</a></li>-->
<!--                  <li class="page-item"><a class="page-link" href="#">&raquo;</a></li>-->
<!--                </ul>-->
<!--              </nav>-->
<!--            </div>-->
          </div>
        </div>
      </div>
      <div class="col">
        <div v-if="!isConnected">
          <button type="button" class="btn btn-primary" v-on:click="connectWallet"> Connect Wallet</button>
        </div>

        <div class="card" style="width: 18em;" v-if="isConnected">
          <h5 class="card-header">Investment</h5>
          <div class="card-body">
            <h5 class="text-end">{{ userAddress }}</h5>
            <h6 class="text-end">5 {{ denom }}</h6>
          </div>


          <div class="card-footer">
            <button type="button" class="btn btn-primary" style="--bs-btn-padding-y: 0.5rem; --bs-btn-padding-x: 2.5rem; --bs-btn-font-size: 1.0rem;">Invest</button>
            <button type="button" class="btn btn-secondary m-lg-1 align-content-end" v-on:click="disconnect">x</button>
          </div>
        </div>
        <div class="mt-5" v-if="isAdmin">
          <button type="button" class="btn btn-danger" style="--bs-btn-padding-y: 0.5rem; --bs-btn-padding-x: 0.9rem; --bs-btn-font-size: 1.0rem;"> Close Investment</button>
        </div>
      </div>
    </div>
    <div class="mt-4" v-if="lastInvestment.length == 0">
      <div class="card">
        <h5 class="card-header text-start">Last Rounds</h5>
        <div class="card-body">

          <table class="table">
            <thead>
            <tr>
              <th scope="col">Round #</th>
              <th scope="col">Total Investments</th>
              <th scope="col">Winner</th>
            </tr>
            </thead>
            <tbody>
<!--            <tr>-->
<!--              <td>1</td>-->
<!--              <td>1 {{ denom }}</td>-->
<!--              <td>link1rudvf55....daam</td>-->
<!--            </tr>-->
            </tbody>
          </table>
          <div>
            <nav aria-label="Page navigation example">
              <ul class="pagination justify-content-center disabled">
                <li class="page-item"><a class="page-link" href="#">&laquo;</a></li>
                <li class="page-item"><a class="page-link" href="#">1</a></li>
                <li class="page-item"><a class="page-link" href="#">2</a></li>
                <li class="page-item"><a class="page-link" href="#">3</a></li>
                <li class="page-item"><a class="page-link" href="#">&raquo;</a></li>
              </ul>
            </nav>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
/* eslint-disable */
import {Options, Vue} from 'vue-class-component';
import {connectDosiVault, DEFAULT_HDPATH, removeWallet, writeWallet} from "@/libs/suggestChain";
import {readWallet, ConnectedWallet} from "@/libs/suggestChain";
import { ref } from 'vue';
import {querySmartContract} from "@/libs/wasm";
import {finschia_chain_info} from "@/libs/finschia";

@Options({
  name: "CurrentInvest",
  components: {},
  props: [],
  data() {
    return {
      isConnected: false,
      isAdmin: false,
      userAddress: "",
      balance: 0,
      denom: "ucony",
      hdPath: DEFAULT_HDPATH,
      contract: "link10qt8wg0n7z740ssvf3urmvgtjhxpyp74hxqvqt7z226gykuus7eqw3r9w0",
      rest: "http://localhost:1317",

      investment: {
        round: 0,
        total_amount: 0,
        investors: []
      },
      lastInvestment: [],
    };
  },
  async created() {
    // const connected = ref(readWallet(this.hdPath) as ConnectedWallet);
    const connected = ref(readWallet(this.hdPath) as ConnectedWallet);
    this.setConnected(connected.value);

    const investment = await this.getCurrentInvestment();
    this.investment.round = investment.round;
    this.investment.total_amount = investment.total_amount;

    this.investment.investors = await this.getCurrentInvestors();
  },
  methods: {
    connectWallet: async function () {
      // @ts-ignore
      const address = await connectDosiVault(finschia_chain_info);
      if (address && address.startsWith("link1")) {
        this.userAddress = address;
        this.isConnected = true;
        const connectedValue = {
          cosmosAddress: address,
          hdPath: DEFAULT_HDPATH,
        }
        writeWallet(connectedValue, this.hdPath);
      }
    },
    setConnected: function (connected: ConnectedWallet) {
      if (connected.cosmosAddress != undefined && connected.hdPath != undefined) {
        this.isConnected = true;
        this.userAddress = connected.cosmosAddress;
        this.hdPath = connected.hdPath;
      }
    },
    disconnect: function () {
      removeWallet(this.hdPath);
      this.isConnected = false;
    },
    getCurrentInvestment: async function () {
      const res = await querySmartContract(this.rest, this.contract, '{"current_investment":{}}');
      return res.data;
    },
    getCurrentInvestors: async function () {
      const res = await querySmartContract(this.rest, this.contract, '{"current_investors":{}}');
      return res.data.investors;
    },
    invest: async function () {

      return 0;
    }
  },
})

export default class CurrentInvest extends Vue {

}
</script>


<style scoped>

</style>

