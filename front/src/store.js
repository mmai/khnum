import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";
import api from "./lib/api";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    user: JSON.parse(localStorage.getItem("user")) || ""
  },
  mutations: {
    auth_check(state) {
    },
    auth_request(state) {
      state.status = "loading";
    },
    auth_success(state, payload) {
      state.status = "success";
      state.token = payload.token;
      state.user = payload.user;
      state.connected = true;
    },
    auth_error(state) {
      state.status = "error";
    },
    logout(state) {
      //console.log("store-logout")
      state.status = "";
      state.token = "";
      state.connected = false;
    }
  },
  actions: {
    auth_check({ commit }) {
      return new Promise((resolve, reject) => {
        commit("auth_check");
        api
          .auth_check()
          .then(resp => {
            const token = resp.data.value;
            const user = resp.data.user;
            commit("auth_success", { user: user, token: token });
            resolve(resp);
          }, (err) => {
            console.log('auth failed');
            commit("auth_error");
            // reject(err);
          })
          .catch(err => {
            reject(err);
          });
      });
    },
    login({ commit }, user) {
      return new Promise((resolve, reject) => {
        commit("auth_request");
        api
          .login(user)
          .then(resp => {
            const token = resp.data.value;
            const user = resp.data.user;
            commit("auth_success", { user: user, token: token });
            Vue.prototype.$http.defaults.headers.common["X-Auth-Token"] = token;
            resolve(resp);
          })
          .catch(err => {
            this.messageErreur = "Identifiant ou mot de passe invalide";
            commit("auth_error");
            reject(err);
          });
      });
    },
    logout({ commit }, router) {
      return new Promise(resolve => {
        commit("logout");
        api.logout().then(resp => {
          delete axios.defaults.headers.common["X-Auth-Token"];
          router.push("/login").then(resolve);
          resolve(resp);
        });
      });
    }
  }
});
