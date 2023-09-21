import { useState } from "react";
import { Alert, Pressable, Text, TextInput, View } from "react-native";

export default function Calculator() {
  const [sumar, setSumar] = useState("");
  const [restar, setRestar] = useState("");
  const [multi, setMulti] = useState("");
  const [divi, setDivi] = useState("");
  return (
    <View>
      <Text>Sumar</Text>
      <TextInput
        style={{ height: 40 }}
        placeholder="numeros a sumar"
        onChangeText={(newSumar) => setSumar(newSumar)}
        defaultValue={sumar}
      />
      <Pressable
        style={{ backgroundColor: "red" }}
        onPress={() => {
          let [a, b] = sumar.split(" ");
          let res = parseInt(a) + parseInt(b);
          Alert.alert("resultado suma", res.toString());
        }}
      >
        <Text>Sumar</Text>
      </Pressable>

      <Text>Restar</Text>
      <TextInput
        style={{ height: 40 }}
        placeholder="numeros a Restar"
        onChangeText={(newRestar) => setRestar(newRestar)}
        defaultValue={restar}
      />
      <Pressable
        style={{ backgroundColor: "red" }}
        onPress={() => {
          let [a, b] = restar.split(" ");
          let res = parseInt(a) - parseInt(b);
          Alert.alert("resultado suma", res.toString());
        }}
      >
        <Text>restar</Text>
      </Pressable>

      <Text>Multiplicar</Text>
      <TextInput
        style={{ height: 40 }}
        placeholder="numeros a Multiplicar"
        onChangeText={(newMulti) => setMulti(newMulti)}
        defaultValue={multi}
      />
      <Pressable
        style={{ backgroundColor: "red" }}
        onPress={() => {
          let [a, b] = multi.split(" ");
          let res = parseInt(a) * parseInt(b);
          Alert.alert("resultado multiplicación", res.toString());
        }}
      >
        <Text>Multiplicar</Text>
      </Pressable>
      <Text>Division</Text>
      <TextInput
        style={{ height: 40 }}
        placeholder="numeros a Division"
        onChangeText={(newDivision) => setDivi(newDivision)}
        defaultValue={divi}
      />
      <Pressable
        style={{ backgroundColor: "red" }}
        onPress={() => {
          let [a, b] = divi.split(" ");
          b = parseInt(b);
          let res = "";
          if (b == 0) {
            res = "error, multiplicación por 0";
            Alert.alert("resultado división", res);
            return;
          }
          res = (parseInt(a) / parseInt(b)).toString();
          Alert.alert("resultado división", res);
        }}
      >
        <Text>División</Text>
      </Pressable>
    </View>
  );
}
