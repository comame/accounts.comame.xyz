export type apis = {
    "/api/signin-password": [passwordRequest, passwordResponse]
    "/api/signin-session": [requestBase, sessionResponse]
    "/api/signin-continue": [continueRequest, continueResponse],
    "/api/signin-continue-nointeraction-fail": [continueNoSessionRequest, continueNoSessionResponse],
}

export type request<T extends keyof apis> = apis[T][0]
export type response<T extends keyof apis> = apis[T][1]

type sessionResponse =
    | {
          error: "bad_request" | "no_session"
      }
    | {
          user_id: string
          last_auth?: number
      }

type passwordResponse =
    | {
          error: "bad_request" | "invalid_credential"
      }
    | {
          user_id: string
      }

type continueResponse = {
    error: "bad_request" | "no_permission"
} | {
    location: string
}

type requestBase = {
    csrf_token: string
    relying_party_id: string
    user_agent_id: string
}

type passwordRequest = {
    user_id: string
    password: string
} & requestBase

type authenticationMethod = "password" | "google" | "session" | "content"

type continueRequest = {
    login_type: authenticationMethod
    state_id: string
} & requestBase

type continueNoSessionRequest = {
    state_id: string
} & requestBase

type continueNoSessionResponse = {
    error: "bad_request"
} | {
    location: string
}
