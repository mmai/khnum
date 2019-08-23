import Vue from "vue";
import GetTextPlugin from "vue-gettext";
import translations from "./translations.json";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";

Vue.use(GetTextPlugin, {
  availableLanguages: {
    en_US: "American English",
    fr_FR: "Français"
  },
  defaultLanguage: "fr_FR",
  translations: translations
});
Vue.config.productionTip = false;

new Vue({
  router,
  store,

  mounted() {
    store.dispatch("auth_check");
  },

  vuetify,
  render: h => h(App)
}).$mount("#app");
