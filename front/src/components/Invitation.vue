<template>
  <div>
    <h1>Invitation</h1>

    {{ log }}

    <p>{{ msg }}</p>
    <label for="email">Email</label>
    <input v-model="email" /><br />
    <button v-on:click="sendVerificationEmail">Send invitation</button>
  </div>
</template>

<script>
import axios from "axios";
export default {
  name: "Invitation",
  data() {
    return {
      email: ""
    };
  },
  props: {
    msg: String,
    log: String
  },
  methods: {
    sendVerificationEmail: function() {
      var getUrl = window.location;
      var baseUrl = getUrl.protocol + "//" + getUrl.host;

      const params = new URLSearchParams(); //This uses  form encoded
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
          this.log = response;
        })
        .catch(e => {
          this.log = e;
        });
    }
  }
};
</script>
