// /src/components/counter/counter-feature.tsx
import { View, StyleSheet } from "react-native";
import CounterStats from "./counter-stats";
import CounterButtonGroup from "./counter-button-group";

export function CounterFeature() {
  return (
    <>
      <View style={styles.container}>
        <CounterStats />
        <CounterButtonGroup />
      </View>
    </>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'space-between',
  },
  buttonContainer: {
    flex: 1,
    justifyContent: 'flex-end',
    paddingBottom: 24,
  },
});
