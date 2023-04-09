import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:0080",
});

export { api };
