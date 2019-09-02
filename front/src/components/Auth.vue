<template>
  <span v-if="fromConnected">
    {{ getUser.login }}
    <v-btn text v-on:click="logout">Logout</v-btn>
  </span>
  <span v-else>
    <v-btn text to="/registration">Register</v-btn>
    <v-dialog v-model="loginDialog" persistent max-width="600px">
      <template v-slot:activator="{ on }">
        <v-btn color="primary" v-on="on">Login</v-btn>
      </template>

      <v-card>
        <v-toolbar color="primary" dark flat>
          <v-toolbar-title>Login</v-toolbar-title>
          <v-spacer></v-spacer>
        </v-toolbar>
        <v-card-text>
          <v-form>
            <v-text-field
              @focus="loginRemoveError()"
              v-model="username"
              label="Login"
              name="login"
              prepend-icon="person"
              type="text"
              :error="loginShowError"
            ></v-text-field>

            <v-text-field
              @focus="loginRemoveError()"
              v-model="password"
              id="password"
              label="Password"
              name="password"
              prepend-icon="lock"
              type="password"
              :error="loginShowError"
            ></v-text-field>
            <v-alert type="error" v-show="loginShowError">
              {{ loginErrorMessage }}
            </v-alert>
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn @click="close" color="warning">Cancel</v-btn>
          <v-btn @click="login" color="primary">Login</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </span>
</template>

<script>
export default {
  name: "Auth",
  data() {
    return {
      loginDialog: false,
      loginErrorMessage: "",
      username: "",
      password: ""
    };
  },
  computed: {
    loginShowError() {
      return this.loginErrorMessage != "";
    },
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
    close: function() {
      this.loginDialog = false;
    },
    logout: function() {
      this.$store.dispatch("logout").then(() => this.$router.push("/"));
    },
    login: function() {
      this.$store
        .dispatch("login", { username: this.username, password: this.password })
        .then(() => {
          this.loginDialog = false;
          // this.$router.push("/");
        })
        .catch(err => {
          this.loginErrorMessage = err.response.data;
        });
    },
    loginRemoveError: function() {
      this.loginErrorMessage = "";
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
