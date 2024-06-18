import React, { useState } from "react"
import { createActor, icrc7 } from '../declarations/icrc7';
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";

import { useQueryCall } from "service/icrc7"

interface MenuProps {}

const Greeting: React.FC<MenuProps> = ({}) => {
  const [name, setName] = useState("")
  const [greeting, setGreeting] = useState('');
  let actor = icrc7;
  async function handleSubmit(event: React.MouseEvent<HTMLElement>) {
    event.preventDefault();
    const authClient = await AuthClient.create();
    await new Promise((resolve) => {
      authClient.login({
        identityProvider:
                process.env.DFX_NETWORK === "ic"
                ? "https://identity.ic0.app"
                : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943`,
        onSuccess: resolve,
      });
    });
    const identity = authClient.getIdentity();
    const agent = new HttpAgent({ identity });
    actor = createActor(process.env.CANISTER_ID_ICRC7!, {
      agent,
    });
    return false;
  }


//  const { call, data, error, loading } = useQueryCall({
//    refetchOnMount: false,
//    functionName: "greet",
//    args: [name]
//  })

//  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
//    const newName = e.target.value
//    setName(newName)
//  }

  return (
    <div>
      <section>
        <button
//           onClick={call}
        >Meili</button>
        <button
//           onClick={call}
        >Book Portal</button>
        <button
           onClick={handleSubmit}
        >Login</button>
      </section>
    </div>
  )
}

export default Greeting
