import React, { FC, useEffect, useState } from "react"
import { observer } from "mobx-react-lite"
import { colors, spacing } from "../theme"
import { Dimensions, TextStyle, TouchableOpacity, View, ViewStyle, StyleSheet } from "react-native"
import { AppStackScreenProps } from "app/navigators"
import { Button, Icon, Screen, Text } from "app/components"
import { Api } from "app/services/api"
import * as FileSystem from "expo-file-system"
import * as IntentLauncher from "expo-intent-launcher"
import * as Progress from "react-native-progress"
import * as ExpoDocPicker from "expo-document-picker"
import { useNavigation } from "@react-navigation/native"
import RustModule from "../utils/rustModule"

interface DocExplorerScreenProps extends AppStackScreenProps<"DocExplorer"> {}

const api = new Api({
  url: "http://10.0.2.2:3000",
  timeout: 10000,
})

const pickDocument = async (): Promise<{ name: string; uri: string } | null> => {
  try {
    const result = await ExpoDocPicker.getDocumentAsync({})

    if (!result.canceled) {
      const file = result.assets[0]
      const { name, uri } = file

      return {
        uri,
        name,
      }
    }
  } catch (err) {
    console.error(err)
  }
  return null
}

// Checks if gif directory exists. If not, creates it
async function ensureDirExists(directory: string) {
  const dirInfo = await FileSystem.getInfoAsync(directory)
  if (!dirInfo.exists) {
    console.log(`${directory} doesn't exist, creating…`)
    await FileSystem.makeDirectoryAsync(directory, { intermediates: true })
  }
}

/* eslint-disable */
// const getFileNameFromHeaders = (headers: Headers) => {
//   const contentDisposition = headers.get("content-disposition")
//   if (!contentDisposition) {
//     return null
//   }
//
//   const filenameMatch = contentDisposition.match(/filename="?(.+)"/)
//   return filenameMatch ? filenameMatch[1] : null
// }

export const DocExplorerScreen: FC<DocExplorerScreenProps> = observer(function DocExplorerScreen() {
  // Pull in one of our MST stores
  // const { someStore, anotherStore } = useStores()

  type Document = {
    id: string
    name?: string
  }

  // TODO: better state managment for files on disk
  // this is just a hack to rerender upon file upload
  const [docsTrigger, updateDocs] = useState({})

  const [docs, setDocs] = useState<Document[] | undefined>()
  const [downloadProgress, setDownloadProgress] = useState(0)

  const dlProgressCallback: FileSystem.FileSystemNetworkTaskProgressCallback<
    FileSystem.DownloadProgressData
  > = (downloadProgress) => {
    const progress = downloadProgress.totalBytesWritten / downloadProgress.totalBytesExpectedToWrite

    setDownloadProgress(progress)
  }

  /**
   * Downloads the remote content to file system and opens it in native file viewer using
   * expo-intent-launcher. This takes the id of the file and requests the resource from the address
   * configured in the api config.
   */
  const viewFile = async (doc: Document) => {
    const url = `${api.config.url}/docs/${doc.id}`

    await ensureDirExists(FileSystem.cacheDirectory + "documents")
    await ensureDirExists(FileSystem.cacheDirectory + "junk")

    try {
      const fileUri = FileSystem.cacheDirectory + `documents/${doc.name}`
      const downloadResumable = FileSystem.createDownloadResumable(
        url,
        fileUri,
        {},
        dlProgressCallback,
      )
      // download the file and view in the native intent-launcher
      const { uri } =
        (await downloadResumable.downloadAsync()) as FileSystem.FileSystemDownloadResult

      FileSystem.getContentUriAsync(uri).then((cUri) => {
        IntentLauncher.startActivityAsync("android.intent.action.VIEW", {
          data: cUri,
          flags: 1,
        })
      })
    } catch (e) {
      console.error(e)
    }
  }

  useEffect(() => {
    api.getDocsList().then((resp) => {
      if (resp.kind === "ok") {
        setDocs(resp.docs)
      }
    })
  }, [docsTrigger])

  // Pull in navigation via hook
  const navigation = useNavigation()

  const FileItem: FC<{ doc: Document }> = ({ doc }) => {
    // try {
    //   // console.log(RustModule.addNumbers(1, 2))
    // } catch (e) {
    //   console.error(e)
    // }
    return (
      <TouchableOpacity onPress={() => viewFile(doc)} style={$fileItem.container}>
        <Icon icon="components" size={50} color={colors.palette.secondary500} />
        <Text numberOfLines={1} ellipsizeMode="tail" style={$fileItem.text}>
          {doc.name ?? doc.id}
        </Text>
      </TouchableOpacity>
    )
  }

  return (
    <Screen
      style={$root}
      preset="scroll"
      contentContainerStyle={$container}
      safeAreaEdges={["top"]}
    >
      <Text preset="heading" style={$title}>
        My Documents
      </Text>

      <View style={[$item, $rowLayout]}>
        {docs?.map((doc) => (
          <FileItem key={doc.id} doc={doc} />
        ))}
      </View>

      {downloadProgress > 0 && downloadProgress < 1 ? (
        <View style={$progressBarContainer}>
          <Progress.Bar
            progress={downloadProgress}
            color={colors.text}
            borderColor={colors.border}
            height={10}
            width={Dimensions.get("window").width * 0.8}
          />
        </View>
      ) : null}

      <Button onPress={() => navigation.goBack()}>Back to Home</Button>
      <Button
        onPress={async () => {
          const file = await pickDocument()
          if (file) {
            await api.uploadFile(file.name, file.uri)
            updateDocs({})
          }
        }}
      >
        Add a file
      </Button>

      <Button
        onPress={async () => {
          const downloadResumable = FileSystem.createDownloadResumable(
            "http://techslides.com/demos/sample-videos/small.mp4",
            FileSystem.documentDirectory + "small.mp4",
            {},
            dlProgressCallback,
          )
          try {
            const { uri } =
              (await downloadResumable.downloadAsync()) as FileSystem.FileSystemDownloadResult
            console.log(`Finished downloading to ${uri}`)
          } catch (e) {
            console.error(e)
          }
        }}
      >
        Test Download
      </Button>
      <Button
        onPress={async () => {
          let x = await RustModule.addNumbers(1, 2)
          console.log(x)
        }}
      >
        Test Rust call
      </Button>
    </Screen>
  )
})

const $progressBarContainer: ViewStyle = {
  justifyContent: "center",
  flexDirection: "row",
  marginBottom: 10,
}

const $root: ViewStyle = {
  flex: 1,
}

const $title: TextStyle = {
  marginBottom: spacing.sm,
}

const $container: ViewStyle = {
  paddingVertical: spacing.lg,
  paddingHorizontal: spacing.lg,
}

const $item: ViewStyle = {
  backgroundColor: colors.palette.neutral100,
  borderRadius: 8,
  padding: spacing.lg,
  marginVertical: spacing.md,
}

const $rowLayout: ViewStyle = {
  flexDirection: "row",
  flexWrap: "wrap",
}

// const $iconTile: ViewStyle = {
//   width: "33.333%",
//   alignItems: "center",
//   paddingVertical: spacing.xs,
// }

// const $demoIconContainer: ViewStyle = {
//   padding: spacing.xs,
// }

// const $iconTile: ViewStyle = {
//   width: "33.333%",
//   alignItems: "center",
//   paddingVertical: spacing.xs,
// }

// const $iconTileLabel: TextStyle = {
//   marginTop: spacing.xxs,
//   color: colors.textDim,
//   textAlign: "center",
// }

const $fileItem = StyleSheet.create({
  container: {
    alignItems: "center",
    paddingHorizontal: 16,
    paddingVertical: 8,
  },
  text: {
    color: colors.text,
    fontSize: 16,
  },
})
