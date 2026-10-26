// The CommonJS build loads with require, and errors match classes across both builds.
const { test } = require("node:test");
const assert = require("node:assert/strict");

test("require('@caretly/caretly') exposes the API", () => {
  const cjs = require("../dist/index.cjs");
  assert.equal(typeof cjs.Caretly, "function");
  assert.equal(typeof cjs.RenderResult, "function");
  const error = new cjs.RateLimitError(429, "slow", new Headers(), 1);
  assert.ok(error instanceof cjs.RateLimitError);
  assert.ok(error instanceof cjs.APIError);
  assert.ok(error instanceof cjs.CaretlyError);
  assert.ok(!(error instanceof cjs.OverloadedError));
});

test("instanceof works across the ESM and CJS builds", async () => {
  const cjs = require("../dist/index.cjs");
  const esm = await import("../dist/index.js");
  assert.notEqual(cjs.APIError, esm.APIError, "two copies of the classes");
  const fromCjs = new cjs.BadRequestError(400, "bad", new Headers(), null);
  const fromEsm = new esm.TimeoutError("slow");
  assert.ok(fromCjs instanceof esm.BadRequestError);
  assert.ok(fromCjs instanceof esm.APIError);
  assert.ok(fromCjs instanceof esm.CaretlyError);
  assert.ok(!(fromCjs instanceof esm.AuthenticationError));
  assert.ok(fromEsm instanceof cjs.TimeoutError);
  assert.ok(fromEsm instanceof cjs.CaretlyError);
  assert.ok(!(fromEsm instanceof cjs.APIError));
});
