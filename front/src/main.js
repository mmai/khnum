import Vue from "vue";
import GetTextPlugin from "vue-gettext";
import translations from "./translations.json";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import vuetify from "./plugins/vuetify";

// Initialize i18n
let availableLanguages = {
  en_US: "American English",
  fr_FR: "Français"
};
let defaultLanguage = "en_US";
let navigatorLanguage = navigator.language.replace("-", "_");
if (Object.keys(availableLanguages).indexOf(navigatorLanguage) > -1) {
  defaultLanguage = navigatorLanguage;
}

Vue.use(GetTextPlugin, {
  availableLanguages,
  defaultLanguage,
  translations
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
