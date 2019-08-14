<template>
  <v-container>
    <v-layout text-center wrap>
      <v-flex xs12>
        <v-img
          :src="require('../assets/logo.svg')"
          class="my-3"
          contain
          height="200"
        ></v-img>
      </v-flex>

      <v-flex mb-4>
        <h1 class="display-2 font-weight-bold mb-3">
          Welcome to Activue
        </h1>
        <p class="subheading font-weight-regular">
          An example of using Actix with Vue (+ vuex & vuetify)
        </p>
        <h2>
          {{ message }}
        </h2>
      </v-flex>
    </v-layout>
  </v-container>
</template>

<script>
export default {
  name: "home",
  props: {
    action: String
  },
  computed: {
    message() {
      switch (this.action) {
        case "registerOk":
          return "Your acount has been created, you can now log in with your username and password.";
        default:
          return "";
      }
    }
  }
};
</script>
