import { createReactor } from "@ic-reactor/react"
import { canisterId, icrc7, idlFactory } from "declarations/icrc7"

export const { initialize,
               useAuth,
               useQueryCall,
               useUpdateCall,
               useActorState} = createReactor<typeof icrc7>({
                 canisterId,
                 idlFactory,
                 withLocalEnv: true,
                 withProcessEnv: true
               })
