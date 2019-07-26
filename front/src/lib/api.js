import axios from "axios";

export default {
  auth_check: () =>
    new Promise((resolve, reject) =>
      axios
        .get("/api/auth")
        .then(resp => {
          console.log(resp);
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
  login: user => new Promise((resolve, reject) => {
    const params = new URLSearchParams();//This uses  form encoded
    params.append("login", user.username);
    params.append("password", user.password);
    axios.post("/api/auth", params)
      .then((resp) => {
        localStorage.setItem("user", JSON.stringify(user));
        resolve(resp);
      })
      .catch(err => {
        localStorage.removeItem("token");
        reject(err);
      })
}
    ),
  logout: () =>
    new Promise(resolve =>
      axios.delete("/api/auth")
      .then((response) => {
        localStorage.removeItem("user");
        resolve(response);
      })
      // .catch((e) => {
      //   this.log = e;
      // })
    )
};
