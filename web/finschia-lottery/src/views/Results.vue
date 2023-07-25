<template>
  <div class="results">
    <div class="container">
      <H1 class="text-start">Round 1</H1>
      <div class="card">
        <div class="card-header">
          <h5 class="text-start mt-1"> <span>Total Investment: </span> <span>100 CONY</span> </h5>
        </div>
        <div class="card-body">
          <table class="table">
            <thead>
            <tr>
              <th scope="col">#</th>
              <th scope="col">Address</th>
              <th scope="col">Reward</th>
            </tr>
            </thead>
            <tbody>
            <tr>
              <td>1</td>
              <td>link1rudvf55....daam</td>
              <td>5 CONY</td>
            </tr>
            <tr>
              <td>2</td>
              <td>link1rudvf55....daam</td>
              <td>5 CONY</td>
            </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="card mt-5">
        <div class="card-header">
          <h5 class="text-start mt-1">All Investors</h5>
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
            <tr>
              <td>link1rudvf55....daam</td>
              <td>5 CONY</td>
            </tr>
            <tr>
              <td>link1rudvf55....daam</td>
              <td>5 CONY</td>
            </tr>
            </tbody>
          </table>
          <div>
            <nav aria-label="Page navigation example">
              <ul class="pagination justify-content-center">
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
import {Options, Vue} from 'vue-class-component';

@Options({
  name: "Results",
  components: {},
  props: ['round'],
  data() {
    return {};
  },
})
export default class Results extends Vue {

}
</script>

<style scoped>

</style>
