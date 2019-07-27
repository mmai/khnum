<template>
  <span v-if="fromConnected">
    {{ getUser.login }}
    <button v-on:click="logout">Logout</button>
  </span>
  <span v-else>
    <router-link to="/invitation">Get invite</router-link>
    <router-link to="/login">Login</router-link>
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
