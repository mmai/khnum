import axios from "axios";

export default {
  auth_check: () =>
    new Promise((resolve, reject) =>
      axios
        .get("/api/auth")
        .then(resp => {
          const token = resp.data.value;
          const user = resp.data.user;
          localStorage.setItem("token", token);
          localStorage.setItem("user", JSON.stringify(user));
          resolve(resp);
        })
        .catch(err => {
          localStorage.removeItem("token");
          reject(err);
        })
    ),
  login: user =>
    new Promise((resolve, reject) =>
      axios
        .post("/login", { login: user.email, password: user.password })
        .then(resp => {
          const token = resp.data.value;
          const user = resp.data.user;
          localStorage.setItem("token", token);
          localStorage.setItem("user", JSON.stringify(user));
          resolve(resp);
        })
        .catch(err => {
          localStorage.removeItem("token");
          reject(err);
        })
    ),
  logout: () =>
    new Promise(resolve =>
      axios.post("/logout").then(resp => {
        localStorage.removeItem("token");
        localStorage.removeItem("user");
        resolve(resp);
      })
    )
};
