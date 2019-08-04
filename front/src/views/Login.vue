<template>
  <v-app id="inspire">
    <v-content>
      <v-container fluid fill-height>
        <v-layout align-center justify-center>
          <v-flex xs12 sm8 md4>
            <v-card class="elevation-12">
              <v-toolbar color="primary" dark flat>
                <v-toolbar-title>Login form</v-toolbar-title>
                <v-spacer></v-spacer>
              </v-toolbar>
              <v-card-text>
                <v-form>
                  <v-text-field
                    v-on:focus="removeError()"
                    v-model="username"
                    label="Login"
                    name="login"
                    prepend-icon="person"
                    type="text"
                    :error="showError()"
                  ></v-text-field>

                  <v-text-field
                    v-on:focus="removeError()"
                    v-model="password"
                    id="password"
                    label="Password"
                    name="password"
                    prepend-icon="lock"
                    type="password"
                    :error="showError()"
                  ></v-text-field>
                  <v-alert type="error" v-show="showError()">
                    {{ errorMessage }}
                  </v-alert>
                </v-form>
              </v-card-text>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn v-on:click="login" color="primary">Login</v-btn>
              </v-card-actions>
            </v-card>
          </v-flex>
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
export default {
  name: "login",
  data() {
    return {
      drawer: null,
      log: "",
      username: "",
      password: "",
      errorMessage: ""
    };
  },
  methods: {
    showError: function() {
      return this.errorMessage != "";
    },
    removeError: function() {
      this.errorMessage = "";
    },
    login: function() {
      this.$store
        .dispatch("login", { username: this.username, password: this.password })
        .then(() => {
          this.failed = false;
          this.$router.push("/");
        })
        .catch(err => {
          this.errorMessage = err.response.data;
        });
    }
  }
};
</script>
