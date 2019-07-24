<template>
  <div>
    {{ log }}
    <input v-model="username" /><br />
    <input v-model="password" /><br />
    <button v-on:click="login">Login</button>
  </div>
</template>

<script>
import axios from 'axios'
export default {
  name: "login",
  data() {
    return {
      log: "",
      username: "",
      password: ""
    }
  },
  methods: {
    login: function () {
      const params = new URLSearchParams();//This uses  form encoded
      params.append("login", this.username);
      params.append("password", this.password);
      axios.post("/api/auth", params)
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
