import Vue from "vue";
import Vuex from "vuex";
import api from "./lib/api";

Vue.use(Vuex);

// Check if user is known at startup
let user = JSON.parse(localStorage.getItem("user")) || "";

export default new Vuex.Store({
  state: {
    status: "",
    user,
    connected: !!user
  },
  mutations: {
    AUTH_CHECK() {},
    AUTH_REQUEST(state) {
      state.status = "loading";
    },
    AUTH_SUCCESS(state, payload) {
      state.status = "success";
      state.user = payload.user;
      state.connected = true;
    },
    AUTH_ERROR(state) {
      state.status = "error";
    },
    LOGOUT(state) {
      state.status = "";
      state.connected = false;
      state.user = null;
    }
  },
  getters: {
    isConnected: state => {
      return state.connected;
    }
  },
  actions: {
    auth_check({ commit }) {
      commit("AUTH_CHECK");
      api
        .auth_check()
        .then(
          resp => {
            const user = resp.data;
            commit("AUTH_SUCCESS", { user });
          },
          err => {
            commit("AUTH_ERROR");
          }
        )
        .catch(err => {
          commit("AUTH_ERROR");
        });
    },
    login({ commit }, user) {
      commit("AUTH_REQUEST");
      api
        .login(user)
        .then(resp => {
          const user = resp.data;
          commit("AUTH_SUCCESS", { user });
          // Vue.prototype.$http.defaults.headers.common["X-Auth-Token"] = token;
        })
        .catch(err => {
          this.messageErreur = "Identifiant ou mot de passe invalide";
          commit("AUTH_ERROR");
        });
    },
    logout({ commit }, router) {
      commit("LOGOUT");
      api.logout().then(() => {
        router.push("/");
      });
    }
  }
});
