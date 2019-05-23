<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <Registration msg="Please enter your email to receive an Invitation" />
  </div>
</template>

<script>
// @ is an alias to /src
import Registration from "@/components/Registration.vue";

export default {
  name: "home",
  components: {
    Registration
  }
};
</script>
