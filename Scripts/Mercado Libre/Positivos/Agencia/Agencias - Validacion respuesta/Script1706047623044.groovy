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

WebUI.callTestCase(findTestCase('Mercado Libre/Positivos/Oauth/Oauth Carrier'), [:], FailureHandling.STOP_ON_FAILURE)

Response2 = WS.sendRequest(findTestObject('Desarrollo ML/Agencias/Agencias - 200'))

agencias = WS.getElementPropertyValue(Response2, 'agencies')

print(agencias)

WS.verifyResponseStatusCode(Response2, 200)

print(Response2.getResponseText())



// Verificar 'agencies'
	assert agencias, "El campo 'agencies' no está presente o es nulo."

// Verificar 'agency_id' en cada objeto de agencia
	agencias.each { agencia ->
	    assert agencia, "El campo 'agencia' no está presente o es nulo para una agencia."
	
	    def agencyId = agencia.agency_id
	    assert agencyId, "El campo 'agency_id' no está presente o es nulo para una agencia."
	    assert agencyId ==~ /^[A-Za-z0-9]+$/, "El 'agency_id' debe contener solo caracteres alfanuméricos."
	}

// Verificar 'agency_name' en cada objeto de agencia
	agencias.each { agency ->
	    def agencyName = agency.agency_name
	    assert agencyName, "El campo 'agency_name' no está presente o es nulo para una agencia."
	}

// Verificar campos en 'location' en cada objeto de agencia
agencias.each { agency ->
    def location = agency.location
    assert location, "El campo 'location' no está presente o es nulo para una agencia."

    assert location.country_name, "El campo 'country_name' no tiene el nombre del país en el que opera."
    assert location.state_name, "El campo 'state_name' no tiene el nombre del departamento."
    assert location.city_name, "El campo 'city_name' no tiene el nombre de la provincia."
    assert location.street_name, "El campo 'street_name' no tiene dirección."
    assert location.street_number, "El campo 'street_number' no tiene número de direccíon."

    if (location.street_number.trim().equalsIgnoreCase("S/N")) {
        // Es válido, ya que la calle no tiene número
    } else {
        // Verificar si es un número válido (puedes ajustar según tus requisitos)
        assert location.street_number.isNumber(), "El 'street_number' debe ser un número o 'S/N' para calles sin número."
    }

    // Verificar 'zip_code' solo si el país utiliza códigos postales
    if (location.country_name in ['Argentina', 'Chile', 'Uruguay', 'Brasil', 'Perú', 'Colombia', 'México']) {
        assert location.zip_code, "El campo 'zip_code' en 'location' no está presente o es nulo para una agencia en un país que utiliza códigos postales."

        // Validar la longitud del código postal según el país
        def zipCodeLength = location.zip_code.length()
        switch (location.country_name) {
            case 'Argentina':
                assert zipCodeLength == 4, "La longitud del 'zip_code' para Argentina debe ser 4."
                break
            case 'Chile':
            case 'Uruguay':
            case 'Perú':
            case 'Colombia':
                // No aplica (N/A)
                break
            case 'Brasil':
                assert zipCodeLength == 8, "La longitud del 'zip_code' para Brasil debe ser 8."
                break
            case 'México':
                assert zipCodeLength == 5, "La longitud del 'zip_code' para México debe ser 5."
                break
            default:
                assert false, "Validación de longitud del 'zip_code' no implementada para el país: ${location.country_name}."
        }
    }

    // Verificar 'geolocation' en 'location'
    def geolocation = location.geolocation
    assert geolocation, "El campo 'geolocation' en 'location' no está presente o es nulo para una agencia."

    assert geolocation.latitude != null, "El campo 'latitude' en 'geolocation' no está presente o es nulo para una agencia."
    assert geolocation.longitude != null, "El campo 'longitude' en 'geolocation' no está presente o es nulo para una agencia."

}

// Verificar 'package_reception' y 'pickup_availability' en cada objeto de agencia
	agencias.each { agency ->
		def packageReception = agency.package_reception
		def pickupAvailability = agency.pickup_availability
		
		// Verificar que 'package_reception' sea true o false
		assert [true, false].contains(packageReception), "El campo 'package_reception' debe ser true o false para una agencia."
		
		// Verificar 'pickup_availability' solo si 'package_reception' es verdadero
		if (packageReception != null && packageReception) {
			// Verificar que 'pickup_availability' sea true o false
			assert [true, false].contains(pickupAvailability), "El campo 'pickup_availability' debe ser true o false si 'package_reception' es true."
		}
	}


// Verificar 'status' en cada objeto de agencia
agencias.each { agency ->
	assert agency.status, "El campo 'status' no está presente o es nulo para una agencia."

	// Verificar que el 'status' tiene un valor válido
	def validStatusValues = ['ACTIVE', 'INACTIVE', 'REGISTERED', 'ACTIVATION_PENDING']
	assert validStatusValues.contains(agency.status), "El valor del campo 'status' no es válido para una agencia."
	
	// Si el estado es 'INACTIVE', asegurarse de que no se envíe la fecha de desactivación
	if (agency.status == 'INACTIVE') {
		assert !agency.deactivation_date, "El campo 'deactivation_date' no debe estar presente para una agencia 'INACTIVE'."
	}
}

// Verificar 'package_capacity' en cada objeto de agencia si el tipo de agencia es PLACE
agencias.each { agency ->
	def agencyType = agency.agency_type
	def packageCapacity = agency.package_capacity
	
	if (agencyType == 'PLACE') {
		// Verificar si 'package_capacity' está presente y es un valor válido
		assert packageCapacity != null, "El campo 'package_capacity' es obligatorio para agencias de tipo 'PLACE'."
		assert packageCapacity.isNumber(), "El campo 'package_capacity' debe ser un número para agencias de tipo 'PLACE'."
	} else {
		// Si el tipo de agencia no es PLACE, asegurarse de que 'package_capacity' no esté presente
		assert packageCapacity == null, "El campo 'package_capacity' no debe estar presente para agencias de tipo diferente a 'PLACE'."
	}
}
