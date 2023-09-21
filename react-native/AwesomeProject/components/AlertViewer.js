import { useState } from "react";
import { Alert, Pressable, Text, TextInput, View } from "react-native";

export default function AlertViewer() {
  const [text, setText] = useState("");
  return (
    <View>
      <TextInput
        style={{ height: 40 }}
        placeholder="Type here!"
        onChangeText={(newText) => setText(newText)}
        defaultValue={text}
      />
      <Pressable
        style={{ backgroundColor: "red" }}
        onPress={() => Alert.alert("my alert", text)}
      >
        <Text>My pressable</Text>
      </Pressable>
    </View>
  );
}
