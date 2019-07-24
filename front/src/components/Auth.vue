<template>
  <span v-if="this.$store.connected">
    {{ this.$store.username }}
    <router-link to="/logout">Logout</router-link>
  </span>
  <span v-else>
      <router-link to="/invitation">Get invite</router-link>
      <router-link to="/login">Login</router-link>
  </span>
</template>

<script>
import axios from 'axios'
export default {
  name: "Auth",
  data() {
    return {
    }
  },
  props: {
    log: String
  },
  methods: {
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
a {
  color: #42b983;
}

</style>
