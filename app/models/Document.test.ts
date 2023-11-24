import { DocumentModel } from "./Document"

test("can be created", () => {
  const instance = DocumentModel.create({})

  expect(instance).toBeTruthy()
})
