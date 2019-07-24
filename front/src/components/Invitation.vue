<template>
  <div class="login">
    <h1>Invitation</h1>

    
    {{ log }}

    <p>{{ msg }}</p>
    <label for='email'>Email</label>
    <input v-model="email"><br>
    <button v-on:click="sendVerificationEmail">Send invitation</button>
  </div>
</template>

<script>
import axios from 'axios'
export default {
  name: "Invitation",
  data() {
    return {
      email: ""
    }
  },
  props: {
    msg: String,
    log: String
  },
  methods: {
    sendVerificationEmail: function () {
      var getUrl = window.location;
      var baseUrl = getUrl .protocol + "//" + getUrl.host;

      const params = new URLSearchParams();//This uses  form encoded
      params.append("email", this.email);
      params.append("register_url", baseUrl + "/#/register");
      axios.post("/register/request", params)
      // To use json encoded : needs to modify api 
      // axios.post( 'register/request', {
      //     email: this.email,
      // })
      .then((response) => {
        this.msg = "Please check your email.";
        this.email = "";
        this.log = response;
      })
      .catch((e) => {
        this.log = e;
      })
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

.login {
  width: 600px;
  margin: auto;
  border: 1px #ccc solid;
  padding: 0px 30px;
  background-color: #3b6caf;
  color: #fff;
}

.field {
  background: #1e4f8a;
  border: 1px #03306b solid;
  padding: 10px;
  margin: 5px 25px;
  width: 215px;
  color: #fff;
}

.login h1,
p,
.chbox,
.btn {
  margin-left: 25px;
  color: #fff;
}

.btn {
  background-color: #00ccff;
  border: 1px #03306b solid;
  padding: 10px 30px;
  font-weight: bold;
  margin: 25px 25px;
  cursor: pointer;
}

.forgot {
  color: #fff;
}
</style>
