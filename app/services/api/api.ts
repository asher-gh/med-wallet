/**
 * This Api class lets you define an API endpoint and methods to request
 * data and process it.
 *
 * See the [Backend API Integration](https://github.com/infinitered/ignite/blob/master/docs/Backend-API-Integration.md)
 * documentation for more details.
 */
import {
  ApiResponse, // @demo remove-current-line
  ApisauceInstance,
  create,
} from "apisauce"
import Config from "../../config"
import { GeneralApiProblem, getGeneralApiProblem } from "./apiProblem" // @demo remove-current-line
import type { ApiConfig, ApiFeedResponse } from "./api.types"
import type { EpisodeSnapshotIn } from "../../models/Episode" // @demo remove-current-line
import { DocumentSnapshotIn } from "app/models"
import * as FS from "expo-file-system"

/**
 * Configuring the apisauce instance.
 */
export const DEFAULT_API_CONFIG: ApiConfig = {
  url: Config.API_URL,
  timeout: 10000,
}

/**
 * Manages all requests to the API. You can use this class to build out
 * various requests that you need to call from your backend API.
 */
export class Api {
  apisauce: ApisauceInstance
  config: ApiConfig

  /**
   * Set up our API instance. Keep this lightweight!
   */
  constructor(config: ApiConfig = DEFAULT_API_CONFIG) {
    this.config = config
    this.apisauce = create({
      baseURL: this.config.url,
      timeout: this.config.timeout,
      headers: {
        Accept: "application/json",
      },
    })
  }

  // @demo remove-block-start
  /**
   * Gets a list of recent React Native Radio episodes.
   */
  async getEpisodes(): Promise<{ kind: "ok"; episodes: EpisodeSnapshotIn[] } | GeneralApiProblem> {
    // make the api call
    const response: ApiResponse<ApiFeedResponse> = await this.apisauce.get(
      `api.json?rss_url=https%3A%2F%2Ffeeds.simplecast.com%2FhEI_f9Dx`,
    )

    // the typical ways to die when calling an api
    if (!response.ok) {
      const problem = getGeneralApiProblem(response)
      if (problem) return problem
    }

    // transform the data into the format we are expecting
    try {
      const rawData = response.data

      // This is where we transform the data into the shape we expect for our MST model.
      const episodes: EpisodeSnapshotIn[] =
        rawData?.items.map((raw) => ({
          ...raw,
        })) ?? []

      return { kind: "ok", episodes }
    } catch (e) {
      if (__DEV__ && e instanceof Error) {
        console.tron.error?.(`Bad data: ${e.message}\n${response.data}`, e.stack)
      }
      return { kind: "bad-data" }
    }
  }
  // @demo remove-block-end

  async getDocsList(): Promise<{ kind: "ok"; docs: DocumentSnapshotIn[] } | GeneralApiProblem> {
    // make the API call
    // TODO: change this to user's id
    const response: ApiResponse<{ id: string }[]> = await this.apisauce.get("/docs")

    if (!response.ok) {
      const problem = getGeneralApiProblem(response)
      if (problem) return problem
    }

    try {
      const rawData = response.data

      // This is where we transform the data into the shape we expect for our MST model.
      const docs: DocumentSnapshotIn[] =
        rawData?.map((rawData) => ({
          ...rawData,
        })) ?? []

      return { kind: "ok", docs }
    } catch (e) {
      if (__DEV__ && e instanceof Error) {
        console.tron.error?.(`Bad data: ${e.message}\n${response.data}`, e.stack)
      }
      return { kind: "bad-data" }
    }
  }

  // FIX: file name and uri validation
  async uploadFile(fileName: string, fileUri: string) {
    // TODO: get user_id from session

    try {
      const res = await FS.uploadAsync(`${this.apisauce.getBaseURL()}/docs`, fileUri, {
        fieldName: `${fileName}`,
        uploadType: FS.FileSystemUploadType.MULTIPART,
      })
    } catch (err) {
      console.error(err)
    }
  }
}

// Singleton instance of the API for convenience
export const api = new Api()
