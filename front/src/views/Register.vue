<template>
  <div class="register">
    <h1>Register Account</h1>

    {{ log }}

    <p>Please choose a username and a password to finish your registration</p>

    <input v-model="username" /><br />
    <input v-model="password" /><br />
    <button v-on:click="register">Register</button>
  </div>
</template>

<script>
import axios from "axios";
export default {
  name: "register",
  data() {
    return {
      log: "",
      username: "",
      password: ""
    };
  },
  methods: {
    register: function() {
      const params = new URLSearchParams(); //This uses  form encoded
      params.append("username", this.username);
      params.append("password", this.password);
      axios
        .post("/register/validate", params)
        .then(response => {
          this.password = "";
          this.log = response;
        })
        .catch(e => {
          this.log = e;
        });
    }
  }
};
</script>
