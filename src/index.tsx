/* @refresh reload */
import "./App.css";
import { render } from "solid-js/web";
import App from "./App";
import SignUpForm from "./_components/SignUpForm";

render(() => <SignUpForm />, document.getElementById("root") as HTMLElement);
