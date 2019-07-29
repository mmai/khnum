<template>
  <span v-if="fromConnected">
    {{ getUser.login }}
    <v-btn text v-on:click="logout">Logout</v-btn>
  </span>
  <span v-else>
    <v-btn text to="/invitation">Get invite</v-btn>
    <v-btn text to="/login">Login</v-btn>
  </span>
</template>

<script>
export default {
  name: "Auth",
  data() {
    return {};
  },
  computed: {
    fromConnected() {
      // return this.$store.state.connected;
      return this.$store.getters.isConnected;
    },
    getUser() {
      return this.$store.state.user;
    }
  },
  props: {
    log: String
  },
  methods: {
    logout: function() {
      this.$store.dispatch("logout").then(() => this.$router.push("/"));
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
