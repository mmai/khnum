import Vue from "vue";
import Router from "vue-router";
import Home from "./views/Home.vue";

Vue.use(Router);

export default new Router({
  // mode: "history", //mode history : needs to configure webserver to redirect all to index.html
  base: process.env.BASE_URL,
  routes: [
    {
      path: "/",
      name: "home",
      component: Home,
      props: route => ({ action: route.query.action })
    },
    {
      path: "/registration",
      name: "registration",
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () =>
        import(/* webpackChunkName: "register" */ "./views/Registration.vue")
    }
  ]
});
