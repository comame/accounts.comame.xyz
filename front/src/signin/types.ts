export type apis = {
    "/api/signin-password": [passwordRequest, passwordResponse]
    "/api/signin-session": [sessionLoginRequest, sessionResponse]
    "/api/signin-continue-nointeraction-fail": [
        continueNoSessionRequest,
        continueNoSessionResponse
    ]
    "/signin/google": [signinRpRequest, continueResponse]
}

export type request<T extends keyof apis> = apis[T][0]
export type response<T extends keyof apis> = apis[T][1]

type sessionResponse =
    | {
          error: "bad_request" | "no_session"
      }
    | {
          location: string
      }

type passwordResponse =
    | {
          error: "bad_request" | "invalid_credential"
      }
    | {
          location: string
      }

type continueResponse =
    | {
          error: "bad_request" | "no_permission"
      }
    | {
          location: string
      }

type requestBase = {
    csrf_token: string
    relying_party_id: string
    user_agent_id: string
}

type sessionLoginRequest = {
    state_id: string
} & requestBase

type passwordRequest = {
    user_id: string
    password: string
    state_id: string
} & requestBase

type continueNoSessionRequest = {
    state_id: string
} & requestBase

type continueNoSessionResponse =
    | {
          error: "bad_request"
      }
    | {
          location: string
      }

type signinRpRequest = {
    state_id: string
    user_agent_id: string
}
