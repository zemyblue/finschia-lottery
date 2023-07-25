<template>
  <div class="home">
    <CurrentInvest msg="Welcome to Your Vue.js + TypeScript App"/>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import CurrentInvest from "@/views/CurrentInvest.vue"; // @ is an alias to /src

@Options({
  components: {
    CurrentInvest,
  },
})
export default class HomeView extends Vue {}
</script>
