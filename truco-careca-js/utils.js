function eq(a, b) {
  return Object.entries(a).toString() === Object.entries(b).toString();
}

export {
  eq
}