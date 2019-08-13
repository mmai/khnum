<template>
  <Registration msg="Please enter your email to receive an Invitation" />
</template>

<script>
// @ is an alias to /src
import Registration from "@/components/Registration.vue";

export default {
  name: "invitation",
  components: {
    Registration
  }
};
</script>
