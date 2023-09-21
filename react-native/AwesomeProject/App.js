import { StatusBar } from "expo-status-bar";
import { useState } from "react";
import {
  StyleSheet,
  Text,
  View,
  Pressable,
  Alert,
  TextInput,
} from "react-native";
import Calculator from "./components/Calculator";
import AlertViewer from "./components/AlertViewer";

export default function App() {
  return (
    <View style={styles.container}>
      <Text>Parcial Programación</Text>
      <AlertViewer />
      <Calculator />

      <View
        style={{
          marginTop: "10px",
          height: "20px",
          width: "30px",
          borderWidth: 4,
          borderColor: "red",
        }}
      >
        <Text>hello</Text>
        <Text style={{ borderTopWidth: 4, borderColor: "red" }}>hello2</Text>
      </View>

      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#fff",
    alignItems: "center",
    justifyContent: "center",
  },
});
