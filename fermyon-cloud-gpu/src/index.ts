import {
  Config,
  HandleRequest,
  HttpRequest,
  HttpResponse,
  InferencingOptions,
  Llm,
  Router,
} from "@fermyon/spin-sdk";

/** Returns true if a valid authorization header is provided. */
function isValidRequest(headers: Record<string, string>): boolean {
  if (headers["authorization"] === `bearer ${Config.get("auth_token")}`) {
    return true;
  }
  return false;
}

/** Response for unauthorized requests. */
const unauthorized = {
  status: 403,
  body: "Please provide valid authorization header",
};

/** Response for bad requests. */
const badRequest = {
  status: 400,
  body: "Please provide valid JSON in the request body",
};

/** Proxy a call to the Fermyon Serverless AI. */
function proxy<T, R>(
  data: HttpRequest,
  operationType: string,
  exec: (params: T) => R
): HttpResponse {
  if (!isValidRequest(data.headers)) {
    console.log("403 - Unauthorized");
    return unauthorized;
  }
  try {
    let params = data.json() as T;
    let response = exec(params);
    console.log(`200 - ${operationType} successful`);
    return {
      status: 200,
      headers: { "content-type": "text/html" },
      body: JSON.stringify(response),
    };
  } catch (error) {
    console.log("400 - Bad request");
    return badRequest;
  }
}

/** The expected input parameters for an inference request. */
interface InferenceParams {
  model: string;
  prompt: string;
  options?: InferencingOptions;
}

/** Handle the proxying of an inference request. */
function infer(data: HttpRequest): HttpResponse {
  return proxy(data, "Inference", (params: InferenceParams) => {
    return Llm.infer(params.model, params.prompt, params.options);
  });
}

/** The expected input parameters for an embedding request. */
interface EmbeddingParams {
  model: string;
  input: string[];
}

/** Handle the proxying of an embedding request. */
function embed(data: HttpRequest): HttpResponse {
  return proxy(data, "Embedding", (params: EmbeddingParams) => {
    return Llm.generateEmbeddings(params.model, params.input);
  });
}

// Setup routing logic
let router = Router();
router.post("/infer", (_, req) => {
  return infer(req);
});
router.post("/embed", (_, req) => {
  return embed(req);
});

// Entrypoint to Spin app
export const handleRequest: HandleRequest = async function (
  request: HttpRequest
): Promise<HttpResponse> {
  return await router.handleRequest(request, request);
};
