<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Oauth Carrier - 401</name>
   <tag></tag>
   <elementGuidId>6326d6b9-972f-4e92-b206-97e17f58409e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;audience\&quot;: \&quot;agencies\&quot;,\n    \&quot;grant_type\&quot;: \&quot;client_credentials\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>a8dc5fe7-1849-47ee-b8ff-1eb9759685f5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic eyJhbGciOiJIUzUxMiJ9.kkkdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDYwNzQyNDcsImF1ZGllbmNlIjoiYWdlbmNpZXMiLCJub21icmUiOiIzMTk5MjI4In0.VCfAGdakZhXF3nP9poidzgi_CA5R3hmXNqVXkxa8ncfUIY9SS6bQrQF6WB3hOC89qQEHFdwU8_V2X2r7Ut4jEg</value>
      <webElementGuid>25cf2df2-c1f8-4efd-a002-c2164dab8ce4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-apimp.olvacourier.com/oauth/token</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
