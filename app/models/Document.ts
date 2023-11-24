import { Instance, SnapshotIn, SnapshotOut, types } from "mobx-state-tree"
import { withSetPropAction } from "./helpers/withSetPropAction"

/**
 * This represents a Document.
 */
export const DocumentModel = types
  .model("Document")
  .props({
    id: types.identifier,
  })
  .actions(withSetPropAction)
// .views((self) => ({})) // eslint-disable-line @typescript-eslint/no-unused-vars

export interface Document extends Instance<typeof DocumentModel> {}
export interface DocumentSnapshotOut extends SnapshotOut<typeof DocumentModel> {}
export interface DocumentSnapshotIn extends SnapshotIn<typeof DocumentModel> {}
// export const createDocumentDefaultModel = () => types.optional(DocumentModel, {})
