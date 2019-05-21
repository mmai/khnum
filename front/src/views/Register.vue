<template>
  <div class="register">
    <h1>Register Account</h1>

    {{ log }}

    <p>Please enter your email and new password</p>

    <input v-model="email" /><br />
    <input v-model="password" /><br />
    <button v-on:click="register">Register</button>
  </div>
</template>

<script>
function getUrlVars() {
  var vars = {};
  window.location.href.replace(/[?&]+([^=&]+)=([^&]*)/gi, function(m, key, value) {
    vars[key] = value;
  });
  return vars;
}

import axios from 'axios'
export default {
  name: "register",
  data() {
    return {
      log: "",
      email: "",
      password: ""
    }
  },
  methods: {
    register: function () {
      let invitation_id = getUrlVars().id;
      axios.post( 'api/register/' + invitation_id, {
          password: this.password,
      })
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

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}

.register {
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

.register h1,
p,
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
