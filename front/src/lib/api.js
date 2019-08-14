import axios from "axios";

// axios.interceptors.response.use(undefined, function(err) {
//   return new Promise(function(resolve, reject) {
//     if (
//       err.response.status === 401 &&
//       err.config &&
//       !err.config.__isRetryRequest
//     ) {
//       console.log("received 401..");
//       // if you ever get an unauthorized, logout the user
//       // this.$store.dispatch(LOGOUT)
//       // you can also redirect to /login if needed !
//     }
//     // throw err;
//   });
// });

export default {
  auth_check: () =>
    new Promise((resolve, reject) =>
      axios
        .get("/api/auth")
        .then(resp => {
          const user = resp.data;
          // localStorage.setItem("token", token);
          localStorage.setItem("user", JSON.stringify(user));
          resolve(resp);
        })
        .catch(err => {
          localStorage.removeItem("token");
          reject(err);
        })
    ),
  login: user =>
    new Promise((resolve, reject) => {
      const params = new URLSearchParams(); //This uses  form encoded
      params.append("login", user.username);
      params.append("password", user.password);
      axios
        .post("/api/auth", params)
        .then(resp => {
          localStorage.setItem(
            "user",
            JSON.stringify({ ...user, password: "-" })
          );
          resolve(resp);
        })
        .catch(err => {
          localStorage.removeItem("token");
          reject(err);
        });
    }),
  logout: () =>
    new Promise(
      resolve =>
        axios.delete("/api/auth").then(resp => {
          localStorage.removeItem("user");
          resolve(resp);
        })
      // .catch((e) => {
      //   this.log = e;
      // })
    )
};
