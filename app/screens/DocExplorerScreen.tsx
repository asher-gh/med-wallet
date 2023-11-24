import React, { FC, useEffect, useState } from "react"
import { observer } from "mobx-react-lite"
import { colors, spacing } from "../theme"
import { Dimensions, TextStyle, TouchableOpacity, View, ViewStyle, StyleSheet } from "react-native"
import { AppStackScreenProps, navigate } from "app/navigators"
import { Button, Icon, Screen, Text } from "app/components"
import { Api } from "app/services/api"
import * as FileSystem from "expo-file-system"
import * as Progress from "react-native-progress"
import * as ExpoDocPicker from "expo-document-picker"

interface DocExplorerScreenProps extends AppStackScreenProps<"DocExplorer"> {}

const api = new Api({
  url: "http://10.0.2.2:3000",
  timeout: 10000,
})

const pickDocument = async (): Promise<{ name: string; uri: string } | null> => {
  try {
    const result = await ExpoDocPicker.getDocumentAsync({
      // type: "mp4",
    })

    if (!result.canceled) {
      const file = result.assets[0]
      console.log(file)
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

export const DocExplorerScreen: FC<DocExplorerScreenProps> = observer(function DocExplorerScreen() {
  // Pull in one of our MST stores
  // const { someStore, anotherStore } = useStores()
  //

  type Document = {
    id: string
    name?: string
  }

  // TODO: better state managment for files on disk
  // this is just a hack to rerender upon file upload
  const [docsTrigger, updateDocs] = useState({})

  const [docs, setDocs] = useState<Document[] | undefined>()
  const [downloadProgress, setDownloadProgress] = useState(0)

  const callback: FileSystem.FileSystemNetworkTaskProgressCallback<
    FileSystem.DownloadProgressData
  > = (downloadProgress) => {
    const progress = downloadProgress.totalBytesWritten / downloadProgress.totalBytesExpectedToWrite

    setDownloadProgress(progress)
  }

  const downloadResumable = FileSystem.createDownloadResumable(
    "http://techslides.com/demos/sample-videos/small.mp4",
    FileSystem.documentDirectory + "small.mp4",
    {},
    callback,
  )

  useEffect(() => {
    api.getDocsList().then((resp) => {
      if (resp.kind === "ok") {
        setDocs(resp.docs)
      }
    })
  }, [docsTrigger])

  // Pull in navigation via hook
  // const navigation = useNavigation()
  const FileItem: FC<{ doc: Document }> = ({ doc }) => {
    return (
      <TouchableOpacity
        onPress={() => console.log("File pressed", doc)}
        style={$fileItem.container}
      >
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

      <Button onPress={() => navigate("DemoCommunity")}>Back to Home</Button>
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
          try {
            const { uri } =
              (await downloadResumable.downloadAsync()) as FileSystem.FileSystemDownloadResult
            console.log(`Finished downloading to ${uri}`)
          } catch (e) {
            console.error(e)
          }
        }}
      >
        Download
      </Button>

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
    </Screen>
  )
})

const $progressBarContainer: ViewStyle = {
  justifyContent: "center",
  flexDirection: "row",
  marginTop: 10,
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
