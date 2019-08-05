<template>
  <v-container>
    <v-layout text-center wrap>
      <h1>Registration</h1>
      <p>{{ msg }}</p>
      <v-form v-model="valid">
        <v-text-field
          v-model="username"
          :rules="[rules.required]"
          label="Name"
          required
          outlined
        ></v-text-field>
        <v-text-field
          v-model="email"
          :rules="[rules.required, rules.validEmail]"
          :type="'email'"
          label="Email"
          outlined
          required
        ></v-text-field>
        <v-text-field
          v-model="password"
          :append-icon="showPassword ? 'visibility' : 'visibility_off'"
          :rules="[rules.required]"
          :type="showPassword ? 'text' : 'password'"
          name="password1"
          label="Password"
          @click:append="showPassword = !showPassword"
          outlined
        ></v-text-field>
        <v-text-field
          v-model="passwordVerif"
          :append-icon="showPasswordVerif ? 'visibility' : 'visibility_off'"
          :rules="[rules.required, rules.passwordsMatch]"
          :type="showPasswordVerif ? 'text' : 'password'"
          label="Password verification"
          @click:append="showPasswordVerif = !showPasswordVerif"
          outlined
        ></v-text-field>
        <v-btn :disabled="!valid" color="success" @click="sendVerificationEmail"
          >Create account</v-btn
        >
      </v-form>
    </v-layout>
  </v-container>
</template>

<script>
import axios from "axios";
export default {
  name: "Registration",
  data() {
    return {
      valid: true,
      username: "",
      password: "",
      showPassword: false,
      passwordVerif: "",
      showPasswordVerif: false,
      email: "",
      rules: {
        required: v => !!v || "Required.",
        validEmail: value => {
          const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
          return pattern.test(value) || "Invalid e-mail.";
        },
        passwordsMatch: v => v === this.password || "Passwords must match"
      }
    };
  },
  props: {
    msg: String
  },
  methods: {
    sendVerificationEmail: function() {
      if (this.$refs.form.validate()) {
        var getUrl = window.location;
        var baseUrl = getUrl.protocol + "//" + getUrl.host;

        const params = new URLSearchParams(); //This uses  form encoded
        params.append("username", this.login);
        params.append("password", this.password);
        params.append("email", this.email);
        params.append("register_url", baseUrl + "/#/register");
        axios
          .post("/register/request", params)
          // To use json encoded : needs to modify api
          // axios.post( 'register/request', {
          //     email: this.email,
          // })
          .then(response => {
            this.msg = "Please check your email.";
            this.email = "";
            console.log(response);
          })
          .catch(e => {
            console.log(e);
          });
      }
    }
  }
};
</script>
