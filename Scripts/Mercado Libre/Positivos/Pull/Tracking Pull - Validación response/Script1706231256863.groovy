import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Response1 = WS.sendRequest(findTestObject('Desarrollo ML/Oauth/Oauth Carrier - tracking-pull'))

Token = WS.getElementPropertyValue(Response1, 'access_token')

print(Token)

Response2 = WS.sendRequest(findTestObject('Desarrollo ML/Pull Tracking/Pull Tracking de Envíos II', [('base_url') : GlobalVariable.base_url]))

print(Response2.getResponseText())

WS.verifyResponseStatusCode(Response2, 200)

shipment = WS.getElementPropertyValue(Response2, 'shipment')



		assert shipment.id != null, "El campo 'id' es obligatorio."
		assert shipment.tracking_number != null, "El campo 'tracking_number' es obligatorio."
		assert shipment.events != null, "El campo 'events' es obligatorio."
		assert shipment.code != null, "El campo 'code' es obligatorio."
		assert shipment.carrier_code != null, "El campo 'carrier_code' es obligatorio."

		// Validar el campo 'payload'
		if (shipment.payload != null) {
			assert shipment.payload.date != null, "El campo 'payload.date' es obligatorio."
			assert shipment.payload.reason != null, "El campo 'payload.reason' es obligatorio para código 0114."
			assert shipment.payload.agency_id != null || !["0201", "0215"].contains(shipment.code), "El campo 'payload.agency_id' es obligatorio para códigos 0201 y 0215."

			// Validar el campo 'payload.cost'
			if (["0260", "0265"].contains(shipment.code)) {
				assert shipment.payload.cost != null, "El campo 'payload.cost' es obligatorio para códigos 0260 y 0265."
			}

			// Validar el campo 'payload.location'
			if (shipment.payload.location != null) {
				assert shipment.payload.location.country_id != null || shipment.code.startsWith("CBT"), "El campo 'payload.location.country_id' es obligatorio para códigos con prefijo 'CBT'."
				assert shipment.payload.location.facility != null || ["0271", "0273"].contains(shipment.code), "El campo 'payload.location.facility' es obligatorio para códigos 0271 y 0273."

				// Validar el campo 'payload.location.geolocation'
				if (shipment.payload.location.geolocation != null) {
					assert shipment.payload.location.geolocation.latitude != null, "El campo 'payload.location.geolocation.latitude' es obligatorio."
					assert shipment.payload.location.geolocation.longitude != null, "El campo 'payload.location.geolocation.longitude' es obligatorio."
				}
			}

			// Validar el campo 'payload.dimensions'
			if (shipment.payload.dimensions != null && shipment.code == "0260") {
				assert shipment.payload.dimensions.weight != null, "El campo 'payload.dimensions.weight' es obligatorio para código 0260."
			}

			// Validar el campo 'payload.estimated_delivery_date'
			if (shipment.code == "0265") {
				assert shipment.payload.estimated_delivery_date != null, "El campo 'payload.estimated_delivery_date' es obligatorio para código 0265."
			}

			// Validar el campo 'payload.estimated_pickup_date'
			if (["0265", "0133"].contains(shipment.code)) {
				assert shipment.payload.estimated_pickup_date != null, "El campo 'payload.estimated_pickup_date' es obligatorio para códigos 0265 y 0133."
			}
		}

		// Validar el campo 'payload.proof_of_delivery'
		if (shipment.payload != null && shipment.payload.proof_of_delivery != null) {
			assert shipment.payload.proof_of_delivery.receiver_document.type != null, "El campo 'payload.proof_of_delivery.receiver_document.type' es obligatorio."
			assert shipment.payload.proof_of_delivery.receiver_document.number != null, "El campo 'payload.proof_of_delivery.receiver_document.number' es obligatorio."
		}

		// Validar el campo 'payload.driver'
		if (shipment.payload != null && shipment.payload.driver != null && shipment.code == "0133") {
			assert shipment.payload.driver.id != null, "El campo 'payload.driver.id' es obligatorio para código 0133."
			assert shipment.payload.driver.name != null, "El campo 'payload.driver.name' es obligatorio para código 0133."
		}
