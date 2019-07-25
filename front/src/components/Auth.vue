<template>
  <span v-if="this.$store.state.connected">
    {{ this.$store.state.user.login }}
    <button v-on:click="logout">Logout</button>
  </span>
  <span v-else>
      <router-link to="/invitation">Get invite</router-link>
      <router-link to="/login">Login</router-link>
  </span>
</template>

<script>
import axios from "axios"
export default {
  name: "Auth",
  data() {
    return {
      connected: this.$store.connected,
      user: this.$store.user,
    }
  },
  props: {
    log: String
  },
  methods: {
    logout: function () {
      this.$store.dispatch("logout");
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
a {
  color: #42b983;
}

</style>
