<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Agencias - 400</name>
   <tag></tag>
   <elementGuidId>625b78ed-5810-4538-9083-068c86f8233c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDU0NDI5MTEsIm5vbWJyZSI6IjMxOTkyMjgifQ.DVGguTOFUwb0egBkUicDA6KhY0Gqezyk86i0W9fYAWuaqI-19eSSnyYlt3k1ZBq1jZn47FNTrzywn4DShS6l5w</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;country_id\&quot;: \&quot;**\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>eaf0c14e-30c5-47dc-9ebd-353102514fd8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDU1NDgyODksIm5vbWJyZSI6IjMxOTkyMjgifQ.a6ajlrd2huSOSc_pWaZBPRuVuwHg-HQ7xU8T_OBn7QjdqDpLFGlnAMOC57nTFkoEyt1LROGFAkKISH5XZUif-w</value>
      <webElementGuid>9d6d55a2-3e8d-4d5d-b9a0-d2d8b3fce726</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-apimp.olvacourier.com/agencies</restUrl>
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
