<template>
  <div>
    {{ log }}
    <button v-on:click="logout">Logout</button>
  </div>
</template>

<script>
import axios from 'axios'
export default {
  name: "logout",
  data() {
    return {
      log: "",
    }
  },
  methods: {
    logout: function () {
      axios.delete("/api/auth")
      .then((response) => {
        this.password = "";
        this.log = response;
      })
      .catch((e) => {
        this.log = e;
      })
    }
  }
};
</script>
