<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <Login msg="Please enter your email to receive an Invitation" />
  </div>
</template>

<script>
// @ is an alias to /src
import Login from "@/components/Login.vue";

export default {
  name: "home",
  components: {
    Login
  }
};
</script>
