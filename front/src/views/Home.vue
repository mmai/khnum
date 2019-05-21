<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <Invitation msg="Please enter your email to receive an Invitation" />
  </div>
</template>

<script>
// @ is an alias to /src
import Invitation from "@/components/Invitation.vue";

export default {
  name: "home",
  components: {
    Invitation
  }
};
</script>
