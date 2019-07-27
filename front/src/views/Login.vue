<template>
  <div>
    {{ log }}
    <input v-model="username" /><br />
    <input v-model="password" /><br />
    <button v-on:click="login">Login</button>
  </div>
</template>

<script>
export default {
  name: "login",
  data() {
    return {
      log: "",
      username: "",
      password: ""
    };
  },
  methods: {
    login: function() {
      this.$store
        .dispatch("login", { username: this.username, password: this.password })
        .then(() => {
          this.$router.push("/");
        });
    }
  }
};
</script>
